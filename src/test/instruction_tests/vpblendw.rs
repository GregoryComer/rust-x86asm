use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 14, 254, 71], OperandSize::Dword)
}

fn vpblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 760487596, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 14, 190, 172, 30, 84, 45, 30], OperandSize::Dword)
}

fn vpblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 14, 220, 100], OperandSize::Qword)
}

fn vpblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 287745669, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 14, 12, 125, 133, 166, 38, 17, 76], OperandSize::Qword)
}

fn vpblendw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 14, 219, 126], OperandSize::Dword)
}

fn vpblendw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 14, 31, 11], OperandSize::Dword)
}

fn vpblendw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 14, 206, 59], OperandSize::Qword)
}

fn vpblendw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 284498840, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 14, 20, 93, 152, 27, 245, 16, 10], OperandSize::Qword)
}

