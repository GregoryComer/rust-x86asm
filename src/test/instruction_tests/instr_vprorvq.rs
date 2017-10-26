use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 20, 250], OperandSize::Dword)
}

#[test]
fn vprorvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 142, 20, 46], OperandSize::Dword)
}

#[test]
fn vprorvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 1103255054, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 158, 20, 129, 14, 86, 194, 65], OperandSize::Dword)
}

#[test]
fn vprorvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 173, 142, 20, 199], OperandSize::Qword)
}

#[test]
fn vprorvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 783698298, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 213, 133, 20, 44, 205, 122, 73, 182, 46], OperandSize::Qword)
}

#[test]
fn vprorvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RSI, 1337863729, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 155, 20, 174, 49, 46, 190, 79], OperandSize::Qword)
}

#[test]
fn vprorvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 173, 20, 209], OperandSize::Dword)
}

#[test]
fn vprorvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 170, 20, 52, 241], OperandSize::Dword)
}

#[test]
fn vprorvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 2095260893, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 190, 20, 44, 133, 221, 36, 227, 124], OperandSize::Dword)
}

#[test]
fn vprorvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 189, 166, 20, 234], OperandSize::Qword)
}

#[test]
fn vprorvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RBX, 2061862729, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 245, 162, 20, 139, 73, 135, 229, 122], OperandSize::Qword)
}

#[test]
fn vprorvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RAX, 1022005413, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 188, 20, 152, 165, 144, 234, 60], OperandSize::Qword)
}

#[test]
fn vprorvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 201, 20, 225], OperandSize::Dword)
}

#[test]
fn vprorvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1852128127, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 206, 20, 28, 197, 127, 59, 101, 110], OperandSize::Dword)
}

#[test]
fn vprorvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDI, 647377663, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 223, 20, 167, 255, 50, 150, 38], OperandSize::Dword)
}

#[test]
fn vprorvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 141, 197, 20, 226], OperandSize::Qword)
}

#[test]
fn vprorvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 690650676, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 203, 20, 148, 215, 52, 126, 42, 41], OperandSize::Qword)
}

#[test]
fn vprorvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 245, 219, 20, 38], OperandSize::Qword)
}

