use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 222, 221], OperandSize::Dword)
}

#[test]
fn vpmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 813351102, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 222, 44, 181, 190, 192, 122, 48], OperandSize::Dword)
}

#[test]
fn vpmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 222, 195], OperandSize::Qword)
}

#[test]
fn vpmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1958576215, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 222, 156, 67, 87, 128, 189, 116], OperandSize::Qword)
}

#[test]
fn vpmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 222, 199], OperandSize::Dword)
}

#[test]
fn vpmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 222, 24], OperandSize::Dword)
}

#[test]
fn vpmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 222, 192], OperandSize::Qword)
}

#[test]
fn vpmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 287618671, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 222, 156, 137, 111, 182, 36, 17], OperandSize::Qword)
}

#[test]
fn vpmaxub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 137, 222, 234], OperandSize::Dword)
}

#[test]
fn vpmaxub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 222, 60, 122], OperandSize::Dword)
}

#[test]
fn vpmaxub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 93, 138, 222, 218], OperandSize::Qword)
}

#[test]
fn vpmaxub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 2041283812, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 143, 222, 188, 159, 228, 132, 171, 121], OperandSize::Qword)
}

#[test]
fn vpmaxub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 174, 222, 252], OperandSize::Dword)
}

#[test]
fn vpmaxub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 173, 222, 46], OperandSize::Dword)
}

#[test]
fn vpmaxub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 45, 172, 222, 234], OperandSize::Qword)
}

#[test]
fn vpmaxub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 77, 164, 222, 2], OperandSize::Qword)
}

#[test]
fn vpmaxub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 222, 214], OperandSize::Dword)
}

#[test]
fn vpmaxub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDX, 1395178087, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 206, 222, 130, 103, 186, 40, 83], OperandSize::Dword)
}

#[test]
fn vpmaxub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 93, 203, 222, 247], OperandSize::Qword)
}

#[test]
fn vpmaxub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RDI, 101113600, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 85, 193, 222, 151, 0, 223, 6, 6], OperandSize::Qword)
}

