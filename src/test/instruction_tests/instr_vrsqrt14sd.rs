use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 79, 200], OperandSize::Dword)
}

#[test]
fn vrsqrt14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 62339401, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 137, 79, 145, 73, 57, 183, 3], OperandSize::Dword)
}

#[test]
fn vrsqrt14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 139, 79, 206], OperandSize::Qword)
}

#[test]
fn vrsqrt14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1491829086, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 181, 134, 79, 188, 119, 94, 129, 235, 88], OperandSize::Qword)
}

