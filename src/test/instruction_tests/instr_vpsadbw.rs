use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 246, 196], OperandSize::Dword)
}

#[test]
fn vpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 1138479113, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 246, 135, 9, 208, 219, 67], OperandSize::Dword)
}

#[test]
fn vpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 246, 211], OperandSize::Qword)
}

#[test]
fn vpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1639602771, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 246, 44, 117, 83, 90, 186, 97], OperandSize::Qword)
}

#[test]
fn vpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 246, 216], OperandSize::Dword)
}

#[test]
fn vpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 246, 12, 119], OperandSize::Dword)
}

#[test]
fn vpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 246, 198], OperandSize::Qword)
}

#[test]
fn vpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 246, 28, 66], OperandSize::Qword)
}

#[test]
fn vpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 246, 242], OperandSize::Dword)
}

#[test]
fn vpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 265676016, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 246, 60, 245, 240, 228, 213, 15], OperandSize::Dword)
}

#[test]
fn vpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 33, 29, 8, 246, 246], OperandSize::Qword)
}

#[test]
fn vpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 161, 246, 4, 246], OperandSize::Qword)
}

#[test]
fn vpsadbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 246, 219], OperandSize::Dword)
}

#[test]
fn vpsadbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 524963191, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 246, 52, 93, 119, 77, 74, 31], OperandSize::Dword)
}

#[test]
fn vpsadbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 65, 93, 246, 232], OperandSize::Qword)
}

#[test]
fn vpsadbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 160757097, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 13, 32, 246, 188, 191, 105, 245, 148, 9], OperandSize::Qword)
}

#[test]
fn vpsadbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 72, 246, 230], OperandSize::Dword)
}

#[test]
fn vpsadbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 616029024, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 101, 72, 246, 12, 221, 96, 219, 183, 36], OperandSize::Dword)
}

#[test]
fn vpsadbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 1, 109, 72, 246, 245], OperandSize::Qword)
}

#[test]
fn vpsadbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1005026276, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 13, 64, 246, 52, 157, 228, 123, 231, 59], OperandSize::Qword)
}

