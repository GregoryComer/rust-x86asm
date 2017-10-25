use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 197, 14, 30, 216, 121], OperandSize::Dword)
}

fn vpcmpuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 1405097144, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 197, 12, 30, 191, 184, 20, 192, 83, 127], OperandSize::Dword)
}

fn vpcmpuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 2089309945, Some(OperandSize::Qword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 221, 28, 30, 12, 69, 249, 86, 136, 124, 0], OperandSize::Dword)
}

fn vpcmpuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 165, 10, 30, 226, 120], OperandSize::Qword)
}

fn vpcmpuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 845998398, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 197, 13, 30, 148, 216, 62, 233, 108, 50, 20], OperandSize::Qword)
}

fn vpcmpuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 2042301498, Some(OperandSize::Qword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 189, 19, 30, 28, 205, 58, 12, 187, 121, 41], OperandSize::Qword)
}

fn vpcmpuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 45, 30, 235, 57], OperandSize::Dword)
}

fn vpcmpuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 44, 30, 28, 246, 71], OperandSize::Dword)
}

fn vpcmpuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 245, 62, 30, 12, 80, 101], OperandSize::Dword)
}

fn vpcmpuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM28)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 147, 165, 45, 30, 204, 37], OperandSize::Qword)
}

fn vpcmpuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 785777853, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 221, 47, 30, 148, 150, 189, 4, 214, 46, 63], OperandSize::Qword)
}

fn vpcmpuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectDisplaced(RAX, 1242564476, Some(OperandSize::Qword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 165, 49, 30, 136, 124, 7, 16, 74, 122], OperandSize::Qword)
}

fn vpcmpuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 245, 77, 30, 224, 109], OperandSize::Dword)
}

fn vpcmpuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 245, 79, 30, 28, 182, 34], OperandSize::Dword)
}

fn vpcmpuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 237, 92, 30, 20, 187, 42], OperandSize::Dword)
}

fn vpcmpuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM28)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 229, 74, 30, 228, 79], OperandSize::Qword)
}

fn vpcmpuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1106811017, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 213, 68, 30, 156, 122, 137, 152, 248, 65, 59], OperandSize::Qword)
}

fn vpcmpuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 157, 84, 30, 60, 134, 75], OperandSize::Qword)
}

