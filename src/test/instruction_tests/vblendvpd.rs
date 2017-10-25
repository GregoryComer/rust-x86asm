use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vblendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 75, 226, 96], OperandSize::Dword)
}

fn vblendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 1457106992, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 75, 142, 48, 176, 217, 86, 0], OperandSize::Dword)
}

fn vblendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Direct(XMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 75, 221, 112], OperandSize::Qword)
}

fn vblendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 603601996, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 75, 156, 88, 76, 60, 250, 35, 0], OperandSize::Qword)
}

fn vblendvpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 75, 194, 16], OperandSize::Dword)
}

fn vblendvpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 1786844356, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 75, 158, 196, 20, 129, 106, 96], OperandSize::Dword)
}

fn vblendvpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: Some(Direct(YMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 75, 204, 32], OperandSize::Qword)
}

fn vblendvpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1173665311, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 75, 156, 154, 31, 182, 244, 69, 96], OperandSize::Qword)
}

