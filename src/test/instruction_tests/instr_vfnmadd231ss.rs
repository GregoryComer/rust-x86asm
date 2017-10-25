use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 189, 193], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 189, 25], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 189, 192], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 189, 10], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 251, 189, 193], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 1627465644, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 189, 183, 172, 39, 1, 97], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 101, 177, 189, 243], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectDisplaced(RDX, 702020748, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 13, 133, 189, 154, 140, 252, 215, 41], OperandSize::Qword)
}

