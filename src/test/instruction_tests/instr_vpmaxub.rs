use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 222, 243], OperandSize::Dword)
}

#[test]
fn vpmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 927060329, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 222, 188, 134, 105, 209, 65, 55], OperandSize::Dword)
}

#[test]
fn vpmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 222, 213], OperandSize::Qword)
}

#[test]
fn vpmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 222, 23], OperandSize::Qword)
}

#[test]
fn vpmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 222, 194], OperandSize::Dword)
}

#[test]
fn vpmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1536402155, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 222, 28, 149, 235, 162, 147, 91], OperandSize::Dword)
}

#[test]
fn vpmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 222, 201], OperandSize::Qword)
}

#[test]
fn vpmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 1898615799, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 222, 170, 247, 147, 42, 113], OperandSize::Qword)
}

#[test]
fn vpmaxub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 222, 234], OperandSize::Dword)
}

#[test]
fn vpmaxub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1948044134, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 138, 222, 188, 222, 102, 203, 28, 116], OperandSize::Dword)
}

#[test]
fn vpmaxub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 85, 134, 222, 216], OperandSize::Qword)
}

#[test]
fn vpmaxub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 29, 142, 222, 52, 190], OperandSize::Qword)
}

#[test]
fn vpmaxub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 222, 228], OperandSize::Dword)
}

#[test]
fn vpmaxub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 222, 44, 145], OperandSize::Dword)
}

#[test]
fn vpmaxub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 69, 169, 222, 254], OperandSize::Qword)
}

#[test]
fn vpmaxub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 916572393, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 61, 169, 222, 140, 246, 233, 200, 161, 54], OperandSize::Qword)
}

#[test]
fn vpmaxub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 204, 222, 196], OperandSize::Dword)
}

#[test]
fn vpmaxub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 203, 222, 9], OperandSize::Dword)
}

#[test]
fn vpmaxub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 61, 195, 222, 199], OperandSize::Qword)
}

#[test]
fn vpmaxub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(RDI, 1869335461, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 93, 204, 222, 151, 165, 203, 107, 111], OperandSize::Qword)
}

