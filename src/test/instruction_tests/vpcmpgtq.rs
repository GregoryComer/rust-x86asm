use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 55, 193], OperandSize::Dword)
}

fn vpcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 55, 16], OperandSize::Dword)
}

fn vpcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 55, 216], OperandSize::Qword)
}

fn vpcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 1941093908, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 55, 163, 20, 190, 178, 115], OperandSize::Qword)
}

fn vpcmpgtq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 55, 241], OperandSize::Dword)
}

fn vpcmpgtq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 55, 0], OperandSize::Dword)
}

fn vpcmpgtq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 55, 247], OperandSize::Qword)
}

fn vpcmpgtq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RBX, 2127124375, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 55, 171, 151, 87, 201, 126], OperandSize::Qword)
}

fn vpcmpgtq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 13, 55, 201], OperandSize::Dword)
}

fn vpcmpgtq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 721732831, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 13, 55, 182, 223, 196, 4, 43], OperandSize::Dword)
}

fn vpcmpgtq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 29, 55, 12, 192], OperandSize::Dword)
}

fn vpcmpgtq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 141, 15, 55, 200], OperandSize::Qword)
}

fn vpcmpgtq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1219255892, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 133, 14, 55, 36, 77, 84, 94, 172, 72], OperandSize::Qword)
}

fn vpcmpgtq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 17, 55, 46], OperandSize::Qword)
}

fn vpcmpgtq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 42, 55, 206], OperandSize::Dword)
}

fn vpcmpgtq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 47, 55, 19], OperandSize::Dword)
}

fn vpcmpgtq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 699915881, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 62, 55, 164, 250, 105, 222, 183, 41], OperandSize::Dword)
}

fn vpcmpgtq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 149, 35, 55, 248], OperandSize::Qword)
}

fn vpcmpgtq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 165, 38, 55, 11], OperandSize::Qword)
}

fn vpcmpgtq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectDisplaced(RDI, 154536396, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 173, 57, 55, 175, 204, 9, 54, 9], OperandSize::Qword)
}

fn vpcmpgtq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 74, 55, 203], OperandSize::Dword)
}

fn vpcmpgtq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDI, 431995596, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 79, 55, 175, 204, 186, 191, 25], OperandSize::Dword)
}

fn vpcmpgtq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 92, 55, 28, 215], OperandSize::Dword)
}

fn vpcmpgtq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 157, 77, 55, 244], OperandSize::Qword)
}

fn vpcmpgtq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 157, 65, 55, 58], OperandSize::Qword)
}

fn vpcmpgtq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1118949584, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 149, 95, 55, 44, 141, 208, 208, 177, 66], OperandSize::Qword)
}

