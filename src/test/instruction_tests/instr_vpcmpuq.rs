use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 213, 15, 30, 209, 124], OperandSize::Dword)
}

#[test]
fn vpcmpuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EAX, 586971913, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 237, 15, 30, 136, 9, 123, 252, 34, 59], OperandSize::Dword)
}

#[test]
fn vpcmpuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDI, 1307256873, Some(OperandSize::Qword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 29, 30, 167, 41, 40, 235, 77, 87], OperandSize::Dword)
}

#[test]
fn vpcmpuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 221, 10, 30, 226, 94], OperandSize::Qword)
}

#[test]
fn vpcmpuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1204706139, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 245, 6, 30, 12, 93, 91, 91, 206, 71, 51], OperandSize::Qword)
}

#[test]
fn vpcmpuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 30, 30, 60, 207, 81], OperandSize::Qword)
}

#[test]
fn vpcmpuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 197, 41, 30, 233, 43], OperandSize::Dword)
}

#[test]
fn vpcmpuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 205, 44, 30, 46, 114], OperandSize::Dword)
}

#[test]
fn vpcmpuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDI, 264603800, Some(OperandSize::Qword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 229, 63, 30, 143, 152, 136, 197, 15, 28], OperandSize::Dword)
}

#[test]
fn vpcmpuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM14)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 173, 43, 30, 222, 45], OperandSize::Qword)
}

#[test]
fn vpcmpuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 665482959, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 189, 39, 30, 36, 205, 207, 118, 170, 39, 109], OperandSize::Qword)
}

#[test]
fn vpcmpuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 133, 62, 30, 52, 214, 38], OperandSize::Qword)
}

#[test]
fn vpcmpuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 229, 78, 30, 241, 19], OperandSize::Dword)
}

#[test]
fn vpcmpuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 838241021, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 221, 75, 30, 20, 85, 253, 138, 246, 49, 28], OperandSize::Dword)
}

#[test]
fn vpcmpuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDI, 1378692048, Some(OperandSize::Qword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 237, 92, 30, 159, 208, 43, 45, 82, 121], OperandSize::Dword)
}

#[test]
fn vpcmpuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM31)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 237, 74, 30, 239, 23], OperandSize::Qword)
}

#[test]
fn vpcmpuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RCX, 1922453623, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 197, 69, 30, 145, 119, 80, 150, 114, 21], OperandSize::Qword)
}

#[test]
fn vpcmpuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 157, 91, 30, 42, 46], OperandSize::Qword)
}

