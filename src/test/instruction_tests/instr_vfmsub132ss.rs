use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 155, 209], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 631818290, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 155, 160, 50, 200, 168, 37], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 155, 210], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 1388354976, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 155, 131, 160, 157, 192, 82], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 219, 155, 224], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1017418612, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 155, 148, 119, 116, 147, 164, 60], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 190, 155, 229], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 45, 129, 155, 3], OperandSize::Qword)
}

