use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 189, 239], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 189, 38], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 189, 216], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RAX, 1103463620, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 189, 160, 196, 132, 197, 65], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 250, 189, 220], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDX, 712003842, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 189, 178, 2, 81, 112, 42], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 197, 220, 189, 224], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 205, 133, 189, 28, 134], OperandSize::Qword)
}

