use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 25, 237], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(ECX, 866553977, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 25, 129, 121, 144, 166, 51], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM25)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 125, 172, 25, 207], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 170, 25, 20, 194], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 25, 246], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1182438543, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 25, 28, 117, 143, 148, 122, 70], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 205, 25, 203], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(RSI, 412199981, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 25, 174, 45, 172, 145, 24], OperandSize::Qword)
}

