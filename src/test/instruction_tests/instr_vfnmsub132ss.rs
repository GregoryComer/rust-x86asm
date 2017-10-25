use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 159, 208], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 159, 28, 123], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 159, 193], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RSI, 964798774, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 159, 142, 54, 169, 129, 57], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 187, 159, 230], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 159, 36, 218], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 45, 153, 159, 216], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 159, 60, 210], OperandSize::Qword)
}

