use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 211, 36], OperandSize::Dword)
}

#[test]
fn vpsrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 209, 80], OperandSize::Qword)
}

#[test]
fn vpsrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 215, 115], OperandSize::Dword)
}

#[test]
fn vpsrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 115, 215, 48], OperandSize::Qword)
}

#[test]
fn vpsrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 211, 197], OperandSize::Dword)
}

#[test]
fn vpsrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1772312167, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 211, 148, 90, 103, 86, 163, 105], OperandSize::Dword)
}

#[test]
fn vpsrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 211, 245], OperandSize::Qword)
}

#[test]
fn vpsrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RSI, 1774013351, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 211, 134, 167, 75, 189, 105], OperandSize::Qword)
}

#[test]
fn vpsrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 211, 225], OperandSize::Dword)
}

#[test]
fn vpsrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 319338407, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 211, 152, 167, 183, 8, 19], OperandSize::Dword)
}

#[test]
fn vpsrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 211, 231], OperandSize::Qword)
}

#[test]
fn vpsrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 150779237, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 211, 20, 85, 101, 181, 252, 8], OperandSize::Qword)
}

#[test]
fn vpsrlq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 137, 211, 222], OperandSize::Dword)
}

#[test]
fn vpsrlq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 110017160, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 140, 211, 52, 253, 136, 186, 142, 6], OperandSize::Dword)
}

#[test]
fn vpsrlq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 149, 132, 211, 205], OperandSize::Qword)
}

#[test]
fn vpsrlq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1843376753, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 149, 133, 211, 188, 155, 113, 178, 223, 109], OperandSize::Qword)
}

#[test]
fn vpsrlq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 169, 211, 252], OperandSize::Dword)
}

#[test]
fn vpsrlq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 880864919, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 175, 211, 169, 151, 238, 128, 52], OperandSize::Dword)
}

#[test]
fn vpsrlq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 157, 172, 211, 245], OperandSize::Qword)
}

#[test]
fn vpsrlq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 692775909, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 237, 162, 211, 52, 69, 229, 235, 74, 41], OperandSize::Qword)
}

#[test]
fn vpsrlq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 206, 211, 211], OperandSize::Dword)
}

#[test]
fn vpsrlq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1705631303, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 202, 211, 180, 113, 71, 222, 169, 101], OperandSize::Dword)
}

#[test]
fn vpsrlq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 253, 198, 211, 208], OperandSize::Qword)
}

#[test]
fn vpsrlq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 213, 204, 211, 55], OperandSize::Qword)
}

