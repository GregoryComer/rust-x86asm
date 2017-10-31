use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 64, 217], OperandSize::Dword)
}

#[test]
fn vpmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 423632856, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 64, 148, 74, 216, 31, 64, 25], OperandSize::Dword)
}

#[test]
fn vpmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 64, 202], OperandSize::Qword)
}

#[test]
fn vpmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 370696884, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 64, 188, 73, 180, 98, 24, 22], OperandSize::Qword)
}

#[test]
fn vpmulld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 64, 237], OperandSize::Dword)
}

#[test]
fn vpmulld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 1156168124, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 64, 188, 118, 188, 185, 233, 68], OperandSize::Dword)
}

#[test]
fn vpmulld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 64, 242], OperandSize::Qword)
}

#[test]
fn vpmulld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1083560812, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 64, 60, 189, 108, 211, 149, 64], OperandSize::Qword)
}

#[test]
fn vpmulld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 64, 222], OperandSize::Dword)
}

#[test]
fn vpmulld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1302843648, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 64, 132, 183, 0, 209, 167, 77], OperandSize::Dword)
}

#[test]
fn vpmulld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1348131764, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 64, 156, 144, 180, 219, 90, 80], OperandSize::Dword)
}

#[test]
fn vpmulld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 117, 130, 64, 223], OperandSize::Qword)
}

#[test]
fn vpmulld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 118183888, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 93, 141, 64, 151, 208, 87, 11, 7], OperandSize::Qword)
}

#[test]
fn vpmulld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1892784308, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 93, 149, 64, 36, 93, 180, 152, 209, 112], OperandSize::Qword)
}

#[test]
fn vpmulld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 64, 210], OperandSize::Dword)
}

#[test]
fn vpmulld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 64, 60, 209], OperandSize::Dword)
}

#[test]
fn vpmulld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 2105270212, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 188, 64, 128, 196, 223, 123, 125], OperandSize::Dword)
}

#[test]
fn vpmulld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 167, 64, 231], OperandSize::Qword)
}

#[test]
fn vpmulld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 161, 64, 8], OperandSize::Qword)
}

#[test]
fn vpmulld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 182, 64, 20, 145], OperandSize::Qword)
}

#[test]
fn vpmulld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 204, 64, 228], OperandSize::Dword)
}

#[test]
fn vpmulld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 19170641, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 204, 64, 144, 81, 133, 36, 1], OperandSize::Dword)
}

#[test]
fn vpmulld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 816923423, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 219, 64, 60, 69, 31, 67, 177, 48], OperandSize::Dword)
}

#[test]
fn vpmulld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 29, 197, 64, 201], OperandSize::Qword)
}

#[test]
fn vpmulld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 204, 64, 3], OperandSize::Qword)
}

#[test]
fn vpmulld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RSI, 1372971779, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 85, 218, 64, 158, 3, 227, 213, 81], OperandSize::Qword)
}

