use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 63, 201], OperandSize::Dword)
}

#[test]
fn vpmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1750049308, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 63, 172, 153, 28, 162, 79, 104], OperandSize::Dword)
}

#[test]
fn vpmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 63, 229], OperandSize::Qword)
}

#[test]
fn vpmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1029135337, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 63, 172, 250, 233, 91, 87, 61], OperandSize::Qword)
}

#[test]
fn vpmaxud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 63, 192], OperandSize::Dword)
}

#[test]
fn vpmaxud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1068969255, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 63, 132, 113, 39, 45, 183, 63], OperandSize::Dword)
}

#[test]
fn vpmaxud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 63, 238], OperandSize::Qword)
}

#[test]
fn vpmaxud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1950576692, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 63, 172, 81, 52, 112, 67, 116], OperandSize::Qword)
}

#[test]
fn vpmaxud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 63, 240], OperandSize::Dword)
}

#[test]
fn vpmaxud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 522547167, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 63, 174, 223, 111, 37, 31], OperandSize::Dword)
}

#[test]
fn vpmaxud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 155, 63, 9], OperandSize::Dword)
}

#[test]
fn vpmaxud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 117, 137, 63, 212], OperandSize::Qword)
}

#[test]
fn vpmaxud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 1638153860, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 93, 141, 63, 135, 132, 62, 164, 97], OperandSize::Qword)
}

#[test]
fn vpmaxud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 2085839701, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 13, 154, 63, 60, 197, 85, 99, 83, 124], OperandSize::Qword)
}

#[test]
fn vpmaxud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 63, 223], OperandSize::Dword)
}

#[test]
fn vpmaxud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 63, 17], OperandSize::Dword)
}

#[test]
fn vpmaxud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 187, 63, 39], OperandSize::Dword)
}

#[test]
fn vpmaxud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 37, 166, 63, 244], OperandSize::Qword)
}

#[test]
fn vpmaxud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 499952589, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 117, 169, 63, 20, 197, 205, 171, 204, 29], OperandSize::Qword)
}

#[test]
fn vpmaxud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1870225622, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 77, 182, 63, 148, 122, 214, 96, 121, 111], OperandSize::Qword)
}

#[test]
fn vpmaxud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 63, 240], OperandSize::Dword)
}

#[test]
fn vpmaxud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 63, 1], OperandSize::Dword)
}

#[test]
fn vpmaxud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EBX, 1785219214, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 222, 63, 187, 142, 72, 104, 106], OperandSize::Dword)
}

#[test]
fn vpmaxud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 45, 197, 63, 214], OperandSize::Qword)
}

#[test]
fn vpmaxud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 211104939, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 61, 194, 63, 60, 213, 171, 52, 149, 12], OperandSize::Qword)
}

#[test]
fn vpmaxud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 109, 221, 63, 11], OperandSize::Qword)
}

