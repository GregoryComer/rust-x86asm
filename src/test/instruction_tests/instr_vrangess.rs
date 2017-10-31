use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 153, 81, 217, 17], OperandSize::Dword)
}

#[test]
fn vrangess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 138, 81, 6, 122], OperandSize::Dword)
}

#[test]
fn vrangess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM20)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 29, 149, 81, 236, 123], OperandSize::Qword)
}

#[test]
fn vrangess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1304234337, Some(OperandSize::Dword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 13, 134, 81, 172, 74, 97, 9, 189, 77, 121], OperandSize::Qword)
}

