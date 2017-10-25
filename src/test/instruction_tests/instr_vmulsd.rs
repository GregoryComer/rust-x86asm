use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 89, 214], OperandSize::Dword)
}

#[test]
fn vmulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 89, 1], OperandSize::Dword)
}

#[test]
fn vmulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 89, 196], OperandSize::Qword)
}

#[test]
fn vmulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1486777117, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 89, 188, 81, 29, 107, 158, 88], OperandSize::Qword)
}

#[test]
fn vmulsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 231, 223, 89, 251], OperandSize::Dword)
}

#[test]
fn vmulsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 1477772481, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 239, 141, 89, 132, 78, 193, 4, 21, 88], OperandSize::Dword)
}

#[test]
fn vmulsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 143, 214, 89, 239], OperandSize::Qword)
}

#[test]
fn vmulsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 159, 134, 89, 18], OperandSize::Qword)
}

