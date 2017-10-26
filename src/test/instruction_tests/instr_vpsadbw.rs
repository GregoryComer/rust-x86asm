use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 246, 196], OperandSize::Dword)
}

#[test]
fn vpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1162340710, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 246, 20, 253, 102, 233, 71, 69], OperandSize::Dword)
}

#[test]
fn vpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 246, 207], OperandSize::Qword)
}

#[test]
fn vpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 702192189, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 246, 12, 69, 61, 154, 218, 41], OperandSize::Qword)
}

#[test]
fn vpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 246, 251], OperandSize::Dword)
}

#[test]
fn vpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1741940848, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 246, 20, 189, 112, 232, 211, 103], OperandSize::Dword)
}

#[test]
fn vpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 246, 216], OperandSize::Qword)
}

#[test]
fn vpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 246, 18], OperandSize::Qword)
}

#[test]
fn vpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 246, 241], OperandSize::Dword)
}

#[test]
fn vpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 643139562, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 246, 44, 77, 234, 135, 85, 38], OperandSize::Dword)
}

#[test]
fn vpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 49, 53, 8, 246, 250], OperandSize::Qword)
}

#[test]
fn vpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 145, 246, 7], OperandSize::Qword)
}

#[test]
fn vpsadbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 246, 200], OperandSize::Dword)
}

#[test]
fn vpsadbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 1435953352, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 246, 144, 200, 232, 150, 85], OperandSize::Dword)
}

#[test]
fn vpsadbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 32, 246, 222], OperandSize::Qword)
}

#[test]
fn vpsadbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1213495642, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 29, 32, 246, 164, 223, 90, 121, 84, 72], OperandSize::Qword)
}

#[test]
fn vpsadbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 72, 246, 254], OperandSize::Dword)
}

#[test]
fn vpsadbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 109, 72, 246, 26], OperandSize::Dword)
}

#[test]
fn vpsadbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 193, 37, 72, 246, 193], OperandSize::Qword)
}

#[test]
fn vpsadbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 109, 64, 246, 52, 150], OperandSize::Qword)
}

