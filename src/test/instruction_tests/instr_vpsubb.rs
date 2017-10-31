use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 248, 233], OperandSize::Dword)
}

#[test]
fn vpsubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 1190315191, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 248, 137, 183, 196, 242, 70], OperandSize::Dword)
}

#[test]
fn vpsubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 248, 207], OperandSize::Qword)
}

#[test]
fn vpsubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1756165212, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 248, 12, 133, 92, 244, 172, 104], OperandSize::Qword)
}

#[test]
fn vpsubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 248, 212], OperandSize::Dword)
}

#[test]
fn vpsubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1458754516, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 248, 4, 181, 212, 211, 242, 86], OperandSize::Dword)
}

#[test]
fn vpsubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 248, 207], OperandSize::Qword)
}

#[test]
fn vpsubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RCX, 1464203349, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 248, 161, 85, 248, 69, 87], OperandSize::Qword)
}

#[test]
fn vpsubb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 248, 232], OperandSize::Dword)
}

#[test]
fn vpsubb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 138, 248, 23], OperandSize::Dword)
}

#[test]
fn vpsubb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 101, 133, 248, 230], OperandSize::Qword)
}

#[test]
fn vpsubb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 102866020, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 37, 133, 248, 172, 223, 100, 156, 33, 6], OperandSize::Qword)
}

#[test]
fn vpsubb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 248, 219], OperandSize::Dword)
}

#[test]
fn vpsubb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 248, 57], OperandSize::Dword)
}

#[test]
fn vpsubb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 101, 175, 248, 216], OperandSize::Qword)
}

#[test]
fn vpsubb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 109, 173, 248, 43], OperandSize::Qword)
}

#[test]
fn vpsubb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 205, 248, 238], OperandSize::Dword)
}

#[test]
fn vpsubb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 2137143623, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 203, 248, 172, 198, 71, 57, 98, 127], OperandSize::Dword)
}

#[test]
fn vpsubb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 45, 207, 248, 255], OperandSize::Qword)
}

#[test]
fn vpsubb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 45, 201, 248, 54], OperandSize::Qword)
}

