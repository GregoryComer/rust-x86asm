use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 155, 222], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1086428415, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 155, 60, 213, 255, 148, 193, 64], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 155, 230], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 155, 20, 219], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 190, 155, 207], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 960503698, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 155, 170, 146, 31, 64, 57], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 53, 154, 155, 241], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDX, 1428128872, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 109, 139, 155, 186, 104, 132, 31, 85], OperandSize::Qword)
}

