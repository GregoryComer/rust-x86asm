use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 159, 210], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 693807047, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 159, 162, 199, 167, 90, 41], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 159, 206], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1098727019, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 159, 60, 181, 107, 62, 125, 65], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 158, 159, 209], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 159, 28, 159], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 53, 222, 159, 206], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 13, 141, 159, 52, 128], OperandSize::Qword)
}

