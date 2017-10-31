use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 191, 45, 222], OperandSize::Dword)
}

#[test]
fn vscalefss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1004331378, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 45, 172, 202, 114, 225, 220, 59], OperandSize::Dword)
}

#[test]
fn vscalefss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 5, 181, 45, 222], OperandSize::Qword)
}

#[test]
fn vscalefss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1347849466, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 45, 141, 45, 36, 85, 250, 140, 86, 80], OperandSize::Qword)
}

