use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 4, 243], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1995266904, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 4, 28, 253, 88, 91, 237, 118], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 4, 205], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDI, 1331100812, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 4, 151, 140, 252, 86, 79], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 4, 199], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1043138911, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 4, 172, 254, 95, 9, 45, 62], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 4, 255], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 4, 39], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 4, 221], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 4, 35], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 13, 141, 4, 193], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 529220125, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 101, 140, 4, 132, 183, 29, 66, 139, 31], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 173, 4, 200], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1592611002, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 170, 4, 4, 141, 186, 80, 237, 94], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 85, 173, 4, 221], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1454554482, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 13, 171, 4, 4, 213, 114, 189, 178, 86], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 207, 4, 217], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1017440417, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 203, 4, 60, 133, 161, 232, 164, 60], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 21, 206, 4, 251], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 301321668, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 13, 202, 4, 12, 181, 196, 205, 245, 17], OperandSize::Qword)
}

