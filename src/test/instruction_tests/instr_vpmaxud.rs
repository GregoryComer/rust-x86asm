use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 63, 221], OperandSize::Dword)
}

#[test]
fn vpmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 63, 52, 206], OperandSize::Dword)
}

#[test]
fn vpmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 63, 221], OperandSize::Qword)
}

#[test]
fn vpmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 467168860, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 63, 155, 92, 110, 216, 27], OperandSize::Qword)
}

#[test]
fn vpmaxud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 63, 196], OperandSize::Dword)
}

#[test]
fn vpmaxud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 63, 62], OperandSize::Dword)
}

#[test]
fn vpmaxud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 63, 248], OperandSize::Qword)
}

#[test]
fn vpmaxud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1630960961, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 63, 28, 149, 65, 125, 54, 97], OperandSize::Qword)
}

#[test]
fn vpmaxud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 63, 196], OperandSize::Dword)
}

#[test]
fn vpmaxud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1133543852, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 139, 63, 172, 114, 172, 129, 144, 67], OperandSize::Dword)
}

#[test]
fn vpmaxud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 63, 39], OperandSize::Dword)
}

#[test]
fn vpmaxud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 69, 141, 63, 223], OperandSize::Qword)
}

#[test]
fn vpmaxud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 21, 134, 63, 60, 243], OperandSize::Qword)
}

#[test]
fn vpmaxud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1801351785, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 117, 145, 63, 28, 69, 105, 114, 94, 107], OperandSize::Qword)
}

#[test]
fn vpmaxud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 169, 63, 199], OperandSize::Dword)
}

#[test]
fn vpmaxud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 63, 6], OperandSize::Dword)
}

#[test]
fn vpmaxud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ECX, 621219221, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 186, 63, 137, 149, 13, 7, 37], OperandSize::Dword)
}

#[test]
fn vpmaxud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 45, 170, 63, 218], OperandSize::Qword)
}

#[test]
fn vpmaxud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RAX, 1505373455, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 5, 162, 63, 160, 15, 45, 186, 89], OperandSize::Qword)
}

#[test]
fn vpmaxud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 2144986571, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 117, 181, 63, 132, 247, 203, 229, 217, 127], OperandSize::Qword)
}

#[test]
fn vpmaxud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 204, 63, 230], OperandSize::Dword)
}

#[test]
fn vpmaxud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ECX, 173155523, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 206, 63, 137, 195, 36, 82, 10], OperandSize::Dword)
}

#[test]
fn vpmaxud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 2124326248, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 217, 63, 163, 104, 165, 158, 126], OperandSize::Dword)
}

#[test]
fn vpmaxud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 37, 202, 63, 238], OperandSize::Qword)
}

#[test]
fn vpmaxud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 69, 197, 63, 38], OperandSize::Qword)
}

#[test]
fn vpmaxud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 938119805, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 109, 221, 63, 60, 125, 125, 146, 234, 55], OperandSize::Qword)
}

