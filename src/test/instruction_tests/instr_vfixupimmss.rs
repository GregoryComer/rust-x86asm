use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 153, 85, 209, 41], OperandSize::Dword)
}

#[test]
fn vfixupimmss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 107205884, Some(OperandSize::Dword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 139, 85, 60, 189, 252, 212, 99, 6, 30], OperandSize::Dword)
}

#[test]
fn vfixupimmss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM17)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 163, 53, 150, 85, 233, 94], OperandSize::Qword)
}

#[test]
fn vfixupimmss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 169252125, Some(OperandSize::Dword), None)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 101, 131, 85, 140, 121, 29, 149, 22, 10, 16], OperandSize::Qword)
}

