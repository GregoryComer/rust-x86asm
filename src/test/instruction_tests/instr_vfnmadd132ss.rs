use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 157, 226], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1203489791, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 157, 148, 79, 255, 203, 187, 71], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 157, 240], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 157, 58], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 188, 157, 254], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 157, 58], OperandSize::Dword)
}

#[test]
fn vfnmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 101, 254, 157, 208], OperandSize::Qword)
}

#[test]
fn vfnmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 13, 129, 157, 50], OperandSize::Qword)
}

