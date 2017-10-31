use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 213, 201], OperandSize::Dword)
}

#[test]
fn vpmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 213, 12, 118], OperandSize::Dword)
}

#[test]
fn vpmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 213, 250], OperandSize::Qword)
}

#[test]
fn vpmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDI, 839221351, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 213, 191, 103, 128, 5, 50], OperandSize::Qword)
}

#[test]
fn vpmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 213, 214], OperandSize::Dword)
}

#[test]
fn vpmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1564963301, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 213, 188, 241, 229, 113, 71, 93], OperandSize::Dword)
}

#[test]
fn vpmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 213, 229], OperandSize::Qword)
}

#[test]
fn vpmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 213, 26], OperandSize::Qword)
}

#[test]
fn vpmullw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 141, 213, 202], OperandSize::Dword)
}

#[test]
fn vpmullw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1172546357, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 213, 148, 242, 53, 163, 227, 69], OperandSize::Dword)
}

#[test]
fn vpmullw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 13, 142, 213, 228], OperandSize::Qword)
}

#[test]
fn vpmullw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RCX, 350100953, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 53, 142, 213, 137, 217, 29, 222, 20], OperandSize::Qword)
}

#[test]
fn vpmullw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 169, 213, 206], OperandSize::Dword)
}

#[test]
fn vpmullw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 213, 19], OperandSize::Dword)
}

#[test]
fn vpmullw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 5, 164, 213, 201], OperandSize::Qword)
}

#[test]
fn vpmullw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM16)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 125, 163, 213, 41], OperandSize::Qword)
}

#[test]
fn vpmullw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 202, 213, 236], OperandSize::Dword)
}

#[test]
fn vpmullw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 768309151, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 213, 12, 85, 159, 119, 203, 45], OperandSize::Dword)
}

#[test]
fn vpmullw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 21, 193, 213, 253], OperandSize::Qword)
}

#[test]
fn vpmullw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 66689098, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 125, 197, 213, 148, 194, 74, 152, 249, 3], OperandSize::Qword)
}

