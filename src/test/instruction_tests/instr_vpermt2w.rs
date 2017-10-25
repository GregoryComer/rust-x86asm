use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 125, 227], OperandSize::Dword)
}

#[test]
fn vpermt2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 2123153823, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 125, 187, 159, 193, 140, 126], OperandSize::Dword)
}

#[test]
fn vpermt2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 133, 131, 125, 234], OperandSize::Qword)
}

#[test]
fn vpermt2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 135, 125, 44, 90], OperandSize::Qword)
}

#[test]
fn vpermt2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 125, 244], OperandSize::Dword)
}

#[test]
fn vpermt2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 8561056, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 125, 20, 85, 160, 161, 130, 0], OperandSize::Dword)
}

#[test]
fn vpermt2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 133, 163, 125, 217], OperandSize::Qword)
}

#[test]
fn vpermt2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1380450116, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 189, 164, 125, 148, 122, 68, 255, 71, 82], OperandSize::Qword)
}

#[test]
fn vpermt2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 125, 216], OperandSize::Dword)
}

#[test]
fn vpermt2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1084860561, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 205, 125, 52, 117, 145, 168, 169, 64], OperandSize::Dword)
}

#[test]
fn vpermt2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 245, 201, 125, 200], OperandSize::Qword)
}

#[test]
fn vpermt2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 2097008347, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 173, 198, 125, 132, 214, 219, 206, 253, 124], OperandSize::Qword)
}

