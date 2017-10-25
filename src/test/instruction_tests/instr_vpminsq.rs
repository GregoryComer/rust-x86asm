use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 57, 254], OperandSize::Dword)
}

#[test]
fn vpminsq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 57, 28, 150], OperandSize::Dword)
}

#[test]
fn vpminsq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 159, 57, 35], OperandSize::Dword)
}

#[test]
fn vpminsq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 181, 141, 57, 203], OperandSize::Qword)
}

#[test]
fn vpminsq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 240044269, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 221, 134, 57, 132, 255, 237, 200, 78, 14], OperandSize::Qword)
}

#[test]
fn vpminsq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 213, 159, 57, 31], OperandSize::Qword)
}

#[test]
fn vpminsq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 173, 57, 241], OperandSize::Dword)
}

#[test]
fn vpminsq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1278345991, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 171, 57, 28, 69, 7, 3, 50, 76], OperandSize::Dword)
}

#[test]
fn vpminsq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1655825657, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 188, 57, 20, 221, 249, 228, 177, 98], OperandSize::Dword)
}

#[test]
fn vpminsq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 237, 172, 57, 238], OperandSize::Qword)
}

#[test]
fn vpminsq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 657904140, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 181, 166, 57, 60, 157, 12, 210, 54, 39], OperandSize::Qword)
}

#[test]
fn vpminsq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 502914481, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 189, 177, 57, 52, 69, 177, 221, 249, 29], OperandSize::Qword)
}

#[test]
fn vpminsq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 57, 202], OperandSize::Dword)
}

#[test]
fn vpminsq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 209319574, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 207, 57, 172, 248, 150, 246, 121, 12], OperandSize::Dword)
}

#[test]
fn vpminsq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 222, 57, 24], OperandSize::Dword)
}

#[test]
fn vpminsq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 141, 198, 57, 221], OperandSize::Qword)
}

#[test]
fn vpminsq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 221, 204, 57, 20, 218], OperandSize::Qword)
}

#[test]
fn vpminsq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 97293481, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 229, 223, 57, 132, 176, 169, 148, 204, 5], OperandSize::Qword)
}

