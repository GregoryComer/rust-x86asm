use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 218, 241], OperandSize::Dword)
}

#[test]
fn vpminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 980832664, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 218, 171, 152, 81, 118, 58], OperandSize::Dword)
}

#[test]
fn vpminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 218, 254], OperandSize::Qword)
}

#[test]
fn vpminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 531732391, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 218, 28, 85, 167, 151, 177, 31], OperandSize::Qword)
}

#[test]
fn vpminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 218, 237], OperandSize::Dword)
}

#[test]
fn vpminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 1924537473, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 218, 159, 129, 28, 182, 114], OperandSize::Dword)
}

#[test]
fn vpminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 218, 245], OperandSize::Qword)
}

#[test]
fn vpminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 218, 36, 75], OperandSize::Qword)
}

#[test]
fn vpminub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 218, 228], OperandSize::Dword)
}

#[test]
fn vpminub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1938805602, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 218, 60, 141, 98, 211, 143, 115], OperandSize::Dword)
}

#[test]
fn vpminub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 109, 129, 218, 228], OperandSize::Qword)
}

#[test]
fn vpminub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 5, 143, 218, 60, 154], OperandSize::Qword)
}

#[test]
fn vpminub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 171, 218, 214], OperandSize::Dword)
}

#[test]
fn vpminub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 170, 218, 22], OperandSize::Dword)
}

#[test]
fn vpminub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 21, 165, 218, 229], OperandSize::Qword)
}

#[test]
fn vpminub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1922305681, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 21, 165, 218, 132, 123, 145, 14, 148, 114], OperandSize::Qword)
}

#[test]
fn vpminub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 207, 218, 217], OperandSize::Dword)
}

#[test]
fn vpminub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 614691881, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 218, 132, 193, 41, 116, 163, 36], OperandSize::Dword)
}

#[test]
fn vpminub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 53, 193, 218, 216], OperandSize::Qword)
}

#[test]
fn vpminub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RDI, 555868280, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 205, 218, 135, 120, 224, 33, 33], OperandSize::Qword)
}

