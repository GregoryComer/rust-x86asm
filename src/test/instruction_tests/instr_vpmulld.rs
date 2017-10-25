use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 64, 245], OperandSize::Dword)
}

#[test]
fn vpmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1876678526, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 64, 140, 182, 126, 215, 219, 111], OperandSize::Dword)
}

#[test]
fn vpmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 64, 220], OperandSize::Qword)
}

#[test]
fn vpmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RBX, 14700989, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 64, 187, 189, 81, 224, 0], OperandSize::Qword)
}

#[test]
fn vpmulld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 64, 247], OperandSize::Dword)
}

#[test]
fn vpmulld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 64, 20, 81], OperandSize::Dword)
}

#[test]
fn vpmulld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 64, 207], OperandSize::Qword)
}

#[test]
fn vpmulld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 64, 20, 158], OperandSize::Qword)
}

#[test]
fn vpmulld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 64, 204], OperandSize::Dword)
}

#[test]
fn vpmulld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1282160611, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 137, 64, 44, 125, 227, 55, 108, 76], OperandSize::Dword)
}

#[test]
fn vpmulld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 2085565304, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 64, 129, 120, 51, 79, 124], OperandSize::Dword)
}

#[test]
fn vpmulld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 13, 130, 64, 254], OperandSize::Qword)
}

#[test]
fn vpmulld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 600573298, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 53, 140, 64, 132, 134, 114, 5, 204, 35], OperandSize::Qword)
}

#[test]
fn vpmulld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1100723897, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 21, 159, 64, 52, 205, 185, 182, 155, 65], OperandSize::Qword)
}

#[test]
fn vpmulld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 64, 202], OperandSize::Dword)
}

#[test]
fn vpmulld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ESI, 152660245, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 64, 190, 21, 105, 25, 9], OperandSize::Dword)
}

#[test]
fn vpmulld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 188, 64, 60, 214], OperandSize::Dword)
}

#[test]
fn vpmulld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 21, 167, 64, 245], OperandSize::Qword)
}

#[test]
fn vpmulld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 170, 64, 31], OperandSize::Qword)
}

#[test]
fn vpmulld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1715989094, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 61, 187, 64, 52, 133, 102, 234, 71, 102], OperandSize::Qword)
}

#[test]
fn vpmulld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 206, 64, 253], OperandSize::Dword)
}

#[test]
fn vpmulld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 207, 64, 49], OperandSize::Dword)
}

#[test]
fn vpmulld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 39920071, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 222, 64, 156, 242, 199, 33, 97, 2], OperandSize::Dword)
}

#[test]
fn vpmulld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 37, 197, 64, 225], OperandSize::Qword)
}

#[test]
fn vpmulld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 576269372, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 77, 207, 64, 164, 131, 60, 44, 89, 34], OperandSize::Qword)
}

#[test]
fn vpmulld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1381904326, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 101, 223, 64, 140, 131, 198, 47, 94, 82], OperandSize::Qword)
}

