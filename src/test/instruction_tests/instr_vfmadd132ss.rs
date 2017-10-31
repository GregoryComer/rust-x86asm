use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 153, 196], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 490216305, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 153, 4, 141, 113, 27, 56, 29], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 153, 253], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 153, 12, 91], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 222, 153, 233], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 81838028, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 153, 156, 153, 204, 191, 224, 4], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 85, 188, 153, 240], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 53, 131, 153, 3], OperandSize::Qword)
}

