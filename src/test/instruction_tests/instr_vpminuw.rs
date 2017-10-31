use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 58, 208], OperandSize::Dword)
}

#[test]
fn vpminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 708112522, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 58, 164, 130, 138, 240, 52, 42], OperandSize::Dword)
}

#[test]
fn vpminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 58, 236], OperandSize::Qword)
}

#[test]
fn vpminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 58, 52, 86], OperandSize::Qword)
}

#[test]
fn vpminuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 58, 202], OperandSize::Dword)
}

#[test]
fn vpminuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 416311935, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 58, 52, 77, 127, 106, 208, 24], OperandSize::Dword)
}

#[test]
fn vpminuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 58, 252], OperandSize::Qword)
}

#[test]
fn vpminuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 1129117081, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 58, 145, 153, 245, 76, 67], OperandSize::Qword)
}

#[test]
fn vpminuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 143, 58, 192], OperandSize::Dword)
}

#[test]
fn vpminuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 451006637, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 58, 148, 214, 173, 208, 225, 26], OperandSize::Dword)
}

#[test]
fn vpminuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 29, 137, 58, 197], OperandSize::Qword)
}

#[test]
fn vpminuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RDI, 1918109289, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 29, 129, 58, 151, 105, 6, 84, 114], OperandSize::Qword)
}

#[test]
fn vpminuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 173, 58, 230], OperandSize::Dword)
}

#[test]
fn vpminuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 58, 52, 82], OperandSize::Dword)
}

#[test]
fn vpminuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 21, 170, 58, 218], OperandSize::Qword)
}

#[test]
fn vpminuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1403535282, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 117, 161, 58, 36, 117, 178, 63, 168, 83], OperandSize::Qword)
}

#[test]
fn vpminuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 201, 58, 230], OperandSize::Dword)
}

#[test]
fn vpminuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1632787026, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 201, 58, 132, 114, 82, 90, 82, 97], OperandSize::Dword)
}

#[test]
fn vpminuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 69, 207, 58, 204], OperandSize::Qword)
}

#[test]
fn vpminuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 203, 58, 26], OperandSize::Qword)
}

