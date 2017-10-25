use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vblendvps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: Some(Direct(XMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 74, 248, 112], OperandSize::Dword)
}

fn vblendvps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 74, 31, 48], OperandSize::Dword)
}

fn vblendvps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: Some(Direct(XMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 74, 248, 48], OperandSize::Qword)
}

fn vblendvps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1206373079, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 74, 44, 221, 215, 202, 231, 71, 48], OperandSize::Qword)
}

fn vblendvps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: Some(Direct(YMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 74, 238, 80], OperandSize::Dword)
}

fn vblendvps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 541399038, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 74, 44, 77, 254, 23, 69, 32, 16], OperandSize::Dword)
}

fn vblendvps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: Some(Direct(YMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 74, 253, 48], OperandSize::Qword)
}

fn vblendvps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDI, 1506761361, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 74, 183, 145, 90, 207, 89, 80], OperandSize::Qword)
}

