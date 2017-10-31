use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 62, 214], OperandSize::Dword)
}

#[test]
fn vpmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 2083605094, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 62, 140, 178, 102, 74, 49, 124], OperandSize::Dword)
}

#[test]
fn vpmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 62, 247], OperandSize::Qword)
}

#[test]
fn vpmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 62, 44, 65], OperandSize::Qword)
}

#[test]
fn vpmaxuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 62, 239], OperandSize::Dword)
}

#[test]
fn vpmaxuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 29320523, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 62, 164, 145, 75, 101, 191, 1], OperandSize::Dword)
}

#[test]
fn vpmaxuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 62, 237], OperandSize::Qword)
}

#[test]
fn vpmaxuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 62, 36, 186], OperandSize::Qword)
}

#[test]
fn vpmaxuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 62, 200], OperandSize::Dword)
}

#[test]
fn vpmaxuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 62, 63], OperandSize::Dword)
}

#[test]
fn vpmaxuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 29, 131, 62, 246], OperandSize::Qword)
}

#[test]
fn vpmaxuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 117, 133, 62, 44, 127], OperandSize::Qword)
}

#[test]
fn vpmaxuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 171, 62, 192], OperandSize::Dword)
}

#[test]
fn vpmaxuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 170, 62, 36, 94], OperandSize::Dword)
}

#[test]
fn vpmaxuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 53, 175, 62, 221], OperandSize::Qword)
}

#[test]
fn vpmaxuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 861789998, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 77, 166, 62, 4, 213, 46, 223, 93, 51], OperandSize::Qword)
}

#[test]
fn vpmaxuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 202, 62, 196], OperandSize::Dword)
}

#[test]
fn vpmaxuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 270221165, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 207, 62, 156, 146, 109, 63, 27, 16], OperandSize::Dword)
}

#[test]
fn vpmaxuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 61, 198, 62, 236], OperandSize::Qword)
}

#[test]
fn vpmaxuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RSI, 1131676434, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 29, 205, 62, 134, 18, 3, 116, 67], OperandSize::Qword)
}

