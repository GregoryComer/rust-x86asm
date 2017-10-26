use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 115, 245, 119], OperandSize::Dword)
}

#[test]
fn vpsllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 115, 241, 104], OperandSize::Qword)
}

#[test]
fn vpsllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 115, 242, 6], OperandSize::Dword)
}

#[test]
fn vpsllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 115, 247, 69], OperandSize::Qword)
}

#[test]
fn vpsllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 243, 196], OperandSize::Dword)
}

#[test]
fn vpsllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 243, 52, 127], OperandSize::Dword)
}

#[test]
fn vpsllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 243, 242], OperandSize::Qword)
}

#[test]
fn vpsllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 243, 10], OperandSize::Qword)
}

#[test]
fn vpsllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 243, 194], OperandSize::Dword)
}

#[test]
fn vpsllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EBX, 982997707, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 243, 171, 203, 90, 151, 58], OperandSize::Dword)
}

#[test]
fn vpsllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 243, 220], OperandSize::Qword)
}

#[test]
fn vpsllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1105477196, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 243, 60, 245, 76, 62, 228, 65], OperandSize::Qword)
}

#[test]
fn vpsllq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 137, 243, 203], OperandSize::Dword)
}

#[test]
fn vpsllq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 2141715734, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 138, 243, 60, 85, 22, 253, 167, 127], OperandSize::Dword)
}

#[test]
fn vpsllq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 173, 139, 243, 223], OperandSize::Qword)
}

#[test]
fn vpsllq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RCX, 1440095636, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 157, 134, 243, 169, 148, 29, 214, 85], OperandSize::Qword)
}

#[test]
fn vpsllq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 174, 243, 237], OperandSize::Dword)
}

#[test]
fn vpsllq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 243, 20, 243], OperandSize::Dword)
}

#[test]
fn vpsllq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 133, 172, 243, 212], OperandSize::Qword)
}

#[test]
fn vpsllq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 141, 163, 243, 12, 199], OperandSize::Qword)
}

#[test]
fn vpsllq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 202, 243, 239], OperandSize::Dword)
}

#[test]
fn vpsllq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1851908656, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 206, 243, 156, 184, 48, 226, 97, 110], OperandSize::Dword)
}

#[test]
fn vpsllq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 197, 193, 243, 239], OperandSize::Qword)
}

#[test]
fn vpsllq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 173, 207, 243, 42], OperandSize::Qword)
}

