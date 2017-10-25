use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 154, 204, 197], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 616513211, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 204, 44, 221, 187, 62, 191, 36], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(ESI, 395529732, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 219, 204, 182, 4, 78, 147, 23], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 253, 155, 204, 212], OperandSize::Qword)
}

#[test]
fn vrsqrt28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM26)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 253, 202, 204, 23], OperandSize::Qword)
}

#[test]
fn vrsqrt28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 221, 204, 28, 126], OperandSize::Qword)
}

