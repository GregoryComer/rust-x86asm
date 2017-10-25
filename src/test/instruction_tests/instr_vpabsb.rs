use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 217], OperandSize::Dword)
}

#[test]
fn vpabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 17], OperandSize::Dword)
}

#[test]
fn vpabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 208], OperandSize::Qword)
}

#[test]
fn vpabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 36, 64], OperandSize::Qword)
}

#[test]
fn vpabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 240], OperandSize::Dword)
}

#[test]
fn vpabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EBX, 1974215055, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 179, 143, 33, 172, 117], OperandSize::Dword)
}

#[test]
fn vpabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 217], OperandSize::Qword)
}

#[test]
fn vpabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RSI, 851080979, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 174, 19, 119, 186, 50], OperandSize::Qword)
}

#[test]
fn vpabsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 28, 222], OperandSize::Dword)
}

#[test]
fn vpabsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 913609661, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 28, 183, 189, 147, 116, 54], OperandSize::Dword)
}

#[test]
fn vpabsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 141, 28, 212], OperandSize::Qword)
}

#[test]
fn vpabsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM14)), operand2: Some(IndirectDisplaced(RBX, 723581299, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 143, 28, 179, 115, 249, 32, 43], OperandSize::Qword)
}

#[test]
fn vpabsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 28, 216], OperandSize::Dword)
}

#[test]
fn vpabsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 28, 58], OperandSize::Dword)
}

#[test]
fn vpabsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 125, 175, 28, 223], OperandSize::Qword)
}

#[test]
fn vpabsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 405943555, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 28, 180, 202, 3, 53, 50, 24], OperandSize::Qword)
}

#[test]
fn vpabsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 28, 212], OperandSize::Dword)
}

#[test]
fn vpabsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(EDX, 1920938514, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 28, 178, 18, 50, 127, 114], OperandSize::Dword)
}

#[test]
fn vpabsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 125, 207, 28, 254], OperandSize::Qword)
}

#[test]
fn vpabsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1012532163, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 28, 180, 246, 195, 3, 90, 60], OperandSize::Qword)
}

