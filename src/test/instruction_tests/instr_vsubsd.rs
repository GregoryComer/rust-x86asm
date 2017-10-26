use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 92, 214], OperandSize::Dword)
}

#[test]
fn vsubsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 30672676, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 92, 156, 145, 36, 7, 212, 1], OperandSize::Dword)
}

#[test]
fn vsubsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 92, 193], OperandSize::Qword)
}

#[test]
fn vsubsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDX, 1498513785, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 92, 186, 121, 129, 81, 89], OperandSize::Qword)
}

#[test]
fn vsubsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 239, 154, 92, 240], OperandSize::Dword)
}

#[test]
fn vsubsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 239, 137, 92, 8], OperandSize::Dword)
}

#[test]
fn vsubsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 159, 146, 92, 240], OperandSize::Qword)
}

#[test]
fn vsubsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 740684703, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 130, 92, 172, 200, 159, 243, 37, 44], OperandSize::Qword)
}

