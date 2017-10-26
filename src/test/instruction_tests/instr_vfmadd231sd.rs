use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 185, 246], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 185, 41], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 185, 222], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 185, 9], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 186, 185, 231], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 2101936343, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 185, 172, 223, 215, 0, 73, 125], OperandSize::Dword)
}

#[test]
fn vfmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 165, 252, 185, 228], OperandSize::Qword)
}

#[test]
fn vfmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectDisplaced(RBX, 266192664, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 141, 131, 185, 187, 24, 199, 221, 15], OperandSize::Qword)
}

