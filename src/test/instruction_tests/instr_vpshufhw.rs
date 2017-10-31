use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 198, 121], OperandSize::Dword)
}

#[test]
fn vpshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1283425669, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 156, 95, 133, 133, 127, 76, 127], OperandSize::Dword)
}

#[test]
fn vpshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 229, 7], OperandSize::Qword)
}

#[test]
fn vpshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RCX, 1883487878, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 161, 134, 190, 67, 112, 49], OperandSize::Qword)
}

#[test]
fn vpshufhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 237, 21], OperandSize::Dword)
}

#[test]
fn vpshufhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 28, 208, 9], OperandSize::Dword)
}

#[test]
fn vpshufhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 216, 78], OperandSize::Qword)
}

#[test]
fn vpshufhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 52, 198, 124], OperandSize::Qword)
}

#[test]
fn vpshufhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 112, 199, 55], OperandSize::Dword)
}

#[test]
fn vpshufhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1532474872, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 112, 140, 187, 248, 181, 87, 91, 86], OperandSize::Dword)
}

#[test]
fn vpshufhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM8)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 126, 141, 112, 216, 34], OperandSize::Qword)
}

#[test]
fn vpshufhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1396020449, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 112, 52, 157, 225, 148, 53, 83, 66], OperandSize::Qword)
}

#[test]
fn vpshufhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 112, 217, 40], OperandSize::Dword)
}

#[test]
fn vpshufhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 272149469, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 112, 12, 189, 221, 171, 56, 16, 114], OperandSize::Dword)
}

#[test]
fn vpshufhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM25)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 126, 173, 112, 193, 70], OperandSize::Qword)
}

#[test]
fn vpshufhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 112, 44, 240, 89], OperandSize::Qword)
}

#[test]
fn vpshufhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 112, 239, 32], OperandSize::Dword)
}

#[test]
fn vpshufhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EAX, 933198014, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 112, 160, 190, 120, 159, 55, 78], OperandSize::Dword)
}

#[test]
fn vpshufhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 112, 244, 90], OperandSize::Qword)
}

#[test]
fn vpshufhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM14)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 126, 201, 112, 54, 102], OperandSize::Qword)
}

