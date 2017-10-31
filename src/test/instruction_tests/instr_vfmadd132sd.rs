use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 153, 229], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDX, 850596139, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 153, 162, 43, 17, 179, 50], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 153, 208], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1690864165, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 153, 172, 136, 37, 138, 200, 100], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 155, 153, 199], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 153, 12, 95], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 181, 149, 153, 194], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 213, 130, 153, 46], OperandSize::Qword)
}

