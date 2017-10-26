use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 115, 212, 120], OperandSize::Dword)
}

#[test]
fn vpsrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 115, 210, 53], OperandSize::Qword)
}

#[test]
fn vpsrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 115, 213, 63], OperandSize::Dword)
}

#[test]
fn vpsrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 115, 209, 123], OperandSize::Qword)
}

#[test]
fn vpsrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 211, 255], OperandSize::Dword)
}

#[test]
fn vpsrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 211, 25], OperandSize::Dword)
}

#[test]
fn vpsrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 211, 201], OperandSize::Qword)
}

#[test]
fn vpsrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 770156608, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 211, 44, 213, 64, 168, 231, 45], OperandSize::Qword)
}

#[test]
fn vpsrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 211, 210], OperandSize::Dword)
}

#[test]
fn vpsrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 211, 3], OperandSize::Dword)
}

#[test]
fn vpsrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 211, 229], OperandSize::Qword)
}

#[test]
fn vpsrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RAX, 2123263895, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 211, 152, 151, 111, 142, 126], OperandSize::Qword)
}

#[test]
fn vpsrlq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 139, 211, 235], OperandSize::Dword)
}

#[test]
fn vpsrlq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 2071688806, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 138, 211, 143, 102, 118, 123, 123], OperandSize::Dword)
}

#[test]
fn vpsrlq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 189, 138, 211, 207], OperandSize::Qword)
}

#[test]
fn vpsrlq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1036079998, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 197, 141, 211, 4, 117, 126, 83, 193, 61], OperandSize::Qword)
}

#[test]
fn vpsrlq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 169, 211, 238], OperandSize::Dword)
}

#[test]
fn vpsrlq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 446308689, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 175, 211, 144, 81, 33, 154, 26], OperandSize::Dword)
}

#[test]
fn vpsrlq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 245, 166, 211, 197], OperandSize::Qword)
}

#[test]
fn vpsrlq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 296310988, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 213, 171, 211, 36, 117, 204, 88, 169, 17], OperandSize::Qword)
}

#[test]
fn vpsrlq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 203, 211, 245], OperandSize::Dword)
}

#[test]
fn vpsrlq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1346272593, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 201, 211, 180, 94, 81, 125, 62, 80], OperandSize::Dword)
}

#[test]
fn vpsrlq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 205, 198, 211, 246], OperandSize::Qword)
}

#[test]
fn vpsrlq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1830370155, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 157, 196, 211, 20, 133, 107, 59, 25, 109], OperandSize::Qword)
}

