use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 153, 216], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 2045492943, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 153, 44, 77, 207, 190, 235, 121], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 153, 206], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 153, 16], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 251, 153, 224], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 153, 44, 136], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 5, 243, 153, 217], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RBX, 533395060, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 69, 131, 153, 147, 116, 246, 202, 31], OperandSize::Qword)
}

