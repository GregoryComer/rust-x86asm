use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmullq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 64, 234], OperandSize::Dword)
}

#[test]
fn vpmullq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 2099432411, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 64, 132, 215, 219, 203, 34, 125], OperandSize::Dword)
}

#[test]
fn vpmullq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 157, 64, 34], OperandSize::Dword)
}

#[test]
fn vpmullq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 253, 138, 64, 207], OperandSize::Qword)
}

#[test]
fn vpmullq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 131, 64, 58], OperandSize::Qword)
}

#[test]
fn vpmullq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1305992066, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 221, 155, 64, 132, 120, 130, 219, 215, 77], OperandSize::Qword)
}

#[test]
fn vpmullq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 171, 64, 207], OperandSize::Dword)
}

#[test]
fn vpmullq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 175, 64, 4, 137], OperandSize::Dword)
}

#[test]
fn vpmullq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 836978118, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 185, 64, 178, 198, 69, 227, 49], OperandSize::Dword)
}

#[test]
fn vpmullq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 189, 166, 64, 255], OperandSize::Qword)
}

#[test]
fn vpmullq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 213, 170, 64, 44, 129], OperandSize::Qword)
}

#[test]
fn vpmullq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RBX, 665277983, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 189, 190, 64, 147, 31, 86, 167, 39], OperandSize::Qword)
}

#[test]
fn vpmullq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 201, 64, 246], OperandSize::Dword)
}

#[test]
fn vpmullq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 205, 64, 46], OperandSize::Dword)
}

#[test]
fn vpmullq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDX, 1943856009, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 223, 64, 170, 137, 227, 220, 115], OperandSize::Dword)
}

#[test]
fn vpmullq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 141, 204, 64, 234], OperandSize::Qword)
}

#[test]
fn vpmullq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectDisplaced(RBX, 1543034035, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 173, 196, 64, 147, 179, 212, 248, 91], OperandSize::Qword)
}

#[test]
fn vpmullq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 213, 218, 64, 18], OperandSize::Qword)
}

