use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexp2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 157, 200, 216], OperandSize::Dword)
}

#[test]
fn vexp2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 200, 20, 202], OperandSize::Dword)
}

#[test]
fn vexp2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 217, 200, 12, 201], OperandSize::Dword)
}

#[test]
fn vexp2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 125, 154, 200, 213], OperandSize::Qword)
}

#[test]
fn vexp2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM28)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 203, 200, 35], OperandSize::Qword)
}

#[test]
fn vexp2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 2076787944, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 221, 200, 60, 77, 232, 68, 201, 123], OperandSize::Qword)
}

