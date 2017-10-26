use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 238, 195], OperandSize::Dword)
}

#[test]
fn vpmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 620284158, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 238, 44, 221, 254, 200, 248, 36], OperandSize::Dword)
}

#[test]
fn vpmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 238, 207], OperandSize::Qword)
}

#[test]
fn vpmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 238, 55], OperandSize::Qword)
}

#[test]
fn vpmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 238, 225], OperandSize::Dword)
}

#[test]
fn vpmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1214407407, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 238, 20, 149, 239, 98, 98, 72], OperandSize::Dword)
}

#[test]
fn vpmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 238, 193], OperandSize::Qword)
}

#[test]
fn vpmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1616942951, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 238, 20, 181, 103, 151, 96, 96], OperandSize::Qword)
}

#[test]
fn vpmaxsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 238, 253], OperandSize::Dword)
}

#[test]
fn vpmaxsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 138, 238, 62], OperandSize::Dword)
}

#[test]
fn vpmaxsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 5, 132, 238, 226], OperandSize::Qword)
}

#[test]
fn vpmaxsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RAX, 2126173938, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 45, 129, 238, 152, 242, 214, 186, 126], OperandSize::Qword)
}

#[test]
fn vpmaxsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 238, 238], OperandSize::Dword)
}

#[test]
fn vpmaxsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDX, 996965233, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 172, 238, 138, 113, 123, 108, 59], OperandSize::Dword)
}

#[test]
fn vpmaxsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 53, 171, 238, 235], OperandSize::Qword)
}

#[test]
fn vpmaxsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RSI, 35870134, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 13, 161, 238, 190, 182, 85, 35, 2], OperandSize::Qword)
}

#[test]
fn vpmaxsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 238, 223], OperandSize::Dword)
}

#[test]
fn vpmaxsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 238, 27], OperandSize::Dword)
}

#[test]
fn vpmaxsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 109, 206, 238, 231], OperandSize::Qword)
}

#[test]
fn vpmaxsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 85, 205, 238, 28, 217], OperandSize::Qword)
}

