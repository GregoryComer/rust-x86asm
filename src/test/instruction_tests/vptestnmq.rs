use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vptestnmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 198, 9, 39, 223], OperandSize::Dword)
}

fn vptestnmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1352643749, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 254, 12, 39, 164, 119, 165, 180, 159, 80], OperandSize::Dword)
}

fn vptestnmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 130374447, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 222, 28, 39, 180, 81, 47, 91, 197, 7], OperandSize::Dword)
}

fn vptestnmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 198, 13, 39, 253], OperandSize::Qword)
}

fn vptestnmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 245834644, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 246, 14, 39, 28, 141, 148, 35, 167, 14], OperandSize::Qword)
}

fn vptestnmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 487892694, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 150, 30, 39, 148, 143, 214, 166, 20, 29], OperandSize::Qword)
}

fn vptestnmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 246, 42, 39, 245], OperandSize::Dword)
}

fn vptestnmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 214, 43, 39, 54], OperandSize::Dword)
}

fn vptestnmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1206181968, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 246, 61, 39, 60, 93, 80, 224, 228, 71], OperandSize::Dword)
}

fn vptestnmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 238, 46, 39, 228], OperandSize::Qword)
}

fn vptestnmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 246, 34, 39, 47], OperandSize::Qword)
}

fn vptestnmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 238, 63, 39, 36, 254], OperandSize::Qword)
}

fn vptestnmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 238, 76, 39, 244], OperandSize::Dword)
}

fn vptestnmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 700995582, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 230, 76, 39, 148, 120, 254, 87, 200, 41], OperandSize::Dword)
}

fn vptestnmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ESI, 2019965805, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 246, 89, 39, 182, 109, 59, 102, 120], OperandSize::Dword)
}

fn vptestnmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 182, 78, 39, 220], OperandSize::Qword)
}

fn vptestnmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1356400483, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 246, 65, 39, 20, 157, 99, 7, 217, 80], OperandSize::Qword)
}

fn vptestnmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 222, 83, 39, 50], OperandSize::Qword)
}

