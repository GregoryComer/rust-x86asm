use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 238, 225], OperandSize::Dword)
}

#[test]
fn vpmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 649806154, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 238, 36, 189, 74, 65, 187, 38], OperandSize::Dword)
}

#[test]
fn vpmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 238, 196], OperandSize::Qword)
}

#[test]
fn vpmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1840432339, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 238, 164, 254, 211, 196, 178, 109], OperandSize::Qword)
}

#[test]
fn vpmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 238, 218], OperandSize::Dword)
}

#[test]
fn vpmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 238, 36, 158], OperandSize::Dword)
}

#[test]
fn vpmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 238, 241], OperandSize::Qword)
}

#[test]
fn vpmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 238, 31], OperandSize::Qword)
}

#[test]
fn vpmaxsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 238, 240], OperandSize::Dword)
}

#[test]
fn vpmaxsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1859501810, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 238, 186, 242, 190, 213, 110], OperandSize::Dword)
}

#[test]
fn vpmaxsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 135, 238, 207], OperandSize::Qword)
}

#[test]
fn vpmaxsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 117, 137, 238, 28, 210], OperandSize::Qword)
}

#[test]
fn vpmaxsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 169, 238, 204], OperandSize::Dword)
}

#[test]
fn vpmaxsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 170, 238, 43], OperandSize::Dword)
}

#[test]
fn vpmaxsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 85, 172, 238, 197], OperandSize::Qword)
}

#[test]
fn vpmaxsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectDisplaced(RDI, 2114138739, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 37, 161, 238, 135, 115, 50, 3, 126], OperandSize::Qword)
}

#[test]
fn vpmaxsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 204, 238, 229], OperandSize::Dword)
}

#[test]
fn vpmaxsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 202, 238, 44, 151], OperandSize::Dword)
}

#[test]
fn vpmaxsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 45, 201, 238, 206], OperandSize::Qword)
}

#[test]
fn vpmaxsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 206, 238, 20, 248], OperandSize::Qword)
}

