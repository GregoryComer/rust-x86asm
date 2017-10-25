use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 76, 240, 96], OperandSize::Dword)
}

fn vpblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 76, 62, 112], OperandSize::Dword)
}

fn vpblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 76, 210, 0], OperandSize::Qword)
}

fn vpblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1717656467, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 76, 52, 189, 147, 91, 97, 102, 16], OperandSize::Qword)
}

fn vpblendvb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 76, 207, 16], OperandSize::Dword)
}

fn vpblendvb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1517336242, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 76, 156, 64, 178, 182, 112, 90, 16], OperandSize::Dword)
}

fn vpblendvb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: Some(Direct(YMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 76, 222, 112], OperandSize::Qword)
}

fn vpblendvb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDVB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 76, 44, 249, 112], OperandSize::Qword)
}

