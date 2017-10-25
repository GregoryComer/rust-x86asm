use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 169, 215], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 618264352, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 169, 12, 253, 32, 247, 217, 36], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 169, 219], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 169, 4, 243], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 157, 169, 205], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 169, 26], OperandSize::Dword)
}

#[test]
fn vfmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 61, 241, 169, 235], OperandSize::Qword)
}

#[test]
fn vfmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 77, 138, 169, 52, 219], OperandSize::Qword)
}

