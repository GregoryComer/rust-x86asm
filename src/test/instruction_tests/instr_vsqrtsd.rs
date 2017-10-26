use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 81, 207], OperandSize::Dword)
}

#[test]
fn vsqrtsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 523416678, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 81, 28, 149, 102, 180, 50, 31], OperandSize::Dword)
}

#[test]
fn vsqrtsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 81, 233], OperandSize::Qword)
}

#[test]
fn vsqrtsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 81, 26], OperandSize::Qword)
}

#[test]
fn vsqrtsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 155, 81, 213], OperandSize::Dword)
}

#[test]
fn vsqrtsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1441402093, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 223, 139, 81, 52, 141, 237, 12, 234, 85], OperandSize::Dword)
}

#[test]
fn vsqrtsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 255, 252, 81, 251], OperandSize::Qword)
}

#[test]
fn vsqrtsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 493783917, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 143, 140, 81, 44, 69, 109, 139, 110, 29], OperandSize::Qword)
}

