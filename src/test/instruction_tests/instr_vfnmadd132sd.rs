use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 157, 207], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 157, 33], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 157, 231], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 157, 44, 214], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 251, 157, 231], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1350165583, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 157, 20, 245, 79, 228, 121, 80], OperandSize::Dword)
}

#[test]
fn vfnmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 181, 186, 157, 252], OperandSize::Qword)
}

#[test]
fn vfnmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 1950728241, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 237, 141, 157, 185, 49, 192, 69, 116], OperandSize::Qword)
}

