use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 203], OperandSize::Word)
}

#[test]
fn adc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 31], OperandSize::Word)
}

#[test]
fn adc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 210], OperandSize::Dword)
}

#[test]
fn adc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1370024530, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 12, 149, 82, 234, 168, 81], OperandSize::Dword)
}

#[test]
fn adc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 202], OperandSize::Qword)
}

#[test]
fn adc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 9], OperandSize::Qword)
}

#[test]
fn adc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 209], OperandSize::Qword)
}

#[test]
fn adc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 20, 128], OperandSize::Qword)
}

#[test]
fn adc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 221], OperandSize::Word)
}

#[test]
fn adc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 27], OperandSize::Word)
}

#[test]
fn adc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 236], OperandSize::Dword)
}

#[test]
fn adc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(ESI, Two, 329615413, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 12, 117, 53, 136, 165, 19], OperandSize::Dword)
}

#[test]
fn adc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 204], OperandSize::Qword)
}

#[test]
fn adc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 11], OperandSize::Qword)
}

#[test]
fn adc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 230], OperandSize::Word)
}

#[test]
fn adc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 12], OperandSize::Word)
}

#[test]
fn adc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 206], OperandSize::Dword)
}

#[test]
fn adc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 8], OperandSize::Dword)
}

#[test]
fn adc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 236], OperandSize::Qword)
}

#[test]
fn adc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1436305683, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 28, 197, 19, 73, 156, 85], OperandSize::Qword)
}

#[test]
fn adc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 215], OperandSize::Qword)
}

#[test]
fn adc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RDX, 900410230, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 146, 118, 43, 171, 53], OperandSize::Qword)
}

#[test]
fn adc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 218], OperandSize::Word)
}

#[test]
fn adc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(BX, 250, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 151, 250, 0], OperandSize::Word)
}

#[test]
fn adc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 210], OperandSize::Dword)
}

#[test]
fn adc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 22], OperandSize::Dword)
}

#[test]
fn adc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 218], OperandSize::Qword)
}

#[test]
fn adc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(RDI, 1356709355, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 151, 235, 189, 221, 80], OperandSize::Qword)
}

#[test]
fn adc_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 218], OperandSize::Qword)
}

#[test]
fn adc_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 20, 255], OperandSize::Qword)
}

#[test]
fn adc_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 229], OperandSize::Word)
}

#[test]
fn adc_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 53], OperandSize::Word)
}

#[test]
fn adc_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 207], OperandSize::Dword)
}

#[test]
fn adc_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 28, 198], OperandSize::Dword)
}

#[test]
fn adc_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 253], OperandSize::Qword)
}

#[test]
fn adc_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1770664622, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 60, 85, 174, 50, 138, 105], OperandSize::Qword)
}

#[test]
fn adc_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 225], OperandSize::Word)
}

#[test]
fn adc_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(BX, 21895, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 183, 135, 85], OperandSize::Word)
}

#[test]
fn adc_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 243], OperandSize::Dword)
}

#[test]
fn adc_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 23], OperandSize::Dword)
}

#[test]
fn adc_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 205], OperandSize::Qword)
}

#[test]
fn adc_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RBX, 1768614843, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 155, 187, 235, 106, 105], OperandSize::Qword)
}

#[test]
fn adc_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 217], OperandSize::Qword)
}

#[test]
fn adc_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RBP)), operand2: Some(IndirectDisplaced(RSI, 1738185404, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 19, 174, 188, 154, 154, 103], OperandSize::Qword)
}

#[test]
fn adc_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 121], OperandSize::Word)
}

#[test]
fn adc_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 23], OperandSize::Dword)
}

#[test]
fn adc_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 4], OperandSize::Qword)
}

#[test]
fn adc_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(16546)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 162, 64], OperandSize::Word)
}

#[test]
fn adc_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(17885)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 221, 69], OperandSize::Dword)
}

#[test]
fn adc_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(26272)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 160, 102], OperandSize::Qword)
}

#[test]
fn adc_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1332248565)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 245, 127, 104, 79], OperandSize::Word)
}

#[test]
fn adc_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(2115903009)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 33, 30, 30, 126], OperandSize::Dword)
}

#[test]
fn adc_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(892834582)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 22, 147, 55, 53], OperandSize::Qword)
}

#[test]
fn adc_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1016419200)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 21, 128, 83, 149, 60], OperandSize::Qword)
}

#[test]
fn adc_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 209, 10], OperandSize::Word)
}

#[test]
fn adc_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 13854, Some(OperandSize::Byte), None)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 144, 30, 54, 51], OperandSize::Word)
}

#[test]
fn adc_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 209, 73], OperandSize::Dword)
}

#[test]
fn adc_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(ESI, 826265341, Some(OperandSize::Byte), None)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 150, 253, 206, 63, 49, 106], OperandSize::Dword)
}

#[test]
fn adc_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 209, 55], OperandSize::Qword)
}

#[test]
fn adc_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RDI, 297598854, Some(OperandSize::Byte), None)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 151, 134, 255, 188, 17, 108], OperandSize::Qword)
}

#[test]
fn adc_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 211, 121], OperandSize::Qword)
}

#[test]
fn adc_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RSI, 2023251850, Some(OperandSize::Byte), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 150, 138, 95, 152, 120, 76], OperandSize::Qword)
}

#[test]
fn adc_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CX)), operand2: Some(Literal16(11535)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 209, 15, 45], OperandSize::Word)
}

#[test]
fn adc_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Memory(4776, Some(OperandSize::Word), None)), operand2: Some(Literal16(5986)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 22, 168, 18, 98, 23], OperandSize::Word)
}

#[test]
fn adc_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CX)), operand2: Some(Literal16(13351)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 209, 39, 52], OperandSize::Dword)
}

#[test]
fn adc_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 353409706, Some(OperandSize::Word), None)), operand2: Some(Literal16(16127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 148, 152, 170, 154, 16, 21, 255, 62], OperandSize::Dword)
}

#[test]
fn adc_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(Literal16(19078)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 215, 134, 74], OperandSize::Qword)
}

#[test]
fn adc_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RSI, 1348052814, Some(OperandSize::Word), None)), operand2: Some(Literal16(32156)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 150, 78, 167, 89, 80, 156, 125], OperandSize::Qword)
}

#[test]
fn adc_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Literal32(1128452342)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 215, 246, 208, 66, 67], OperandSize::Word)
}

#[test]
fn adc_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(216420583)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 23, 231, 80, 230, 12], OperandSize::Word)
}

#[test]
fn adc_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Literal32(415688224)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 215, 32, 230, 198, 24], OperandSize::Dword)
}

#[test]
fn adc_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 1187540120, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1537803074)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 20, 205, 152, 108, 200, 70, 66, 3, 169, 91], OperandSize::Dword)
}

#[test]
fn adc_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Literal32(2062392834)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 211, 2, 158, 237, 122], OperandSize::Qword)
}

#[test]
fn adc_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RDX, 406887686, Some(OperandSize::Dword), None)), operand2: Some(Literal32(794944090)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 146, 6, 157, 64, 24, 90, 226, 97, 47], OperandSize::Qword)
}

#[test]
fn adc_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RSI)), operand2: Some(Literal32(1745522583)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 214, 151, 143, 10, 104], OperandSize::Qword)
}

#[test]
fn adc_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RAX, 1826741243, Some(OperandSize::Qword), None)), operand2: Some(Literal32(172865679)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 144, 251, 219, 225, 108, 143, 184, 77, 10], OperandSize::Qword)
}

#[test]
fn adc_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 212, 38], OperandSize::Word)
}

#[test]
fn adc_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 121, Some(OperandSize::Word), None)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 81, 121, 79], OperandSize::Word)
}

#[test]
fn adc_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 212, 38], OperandSize::Dword)
}

#[test]
fn adc_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 20, 114, 17], OperandSize::Dword)
}

#[test]
fn adc_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BX)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 211, 16], OperandSize::Qword)
}

#[test]
fn adc_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1096567149, Some(OperandSize::Word), None)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 20, 213, 109, 73, 92, 65, 74], OperandSize::Qword)
}

#[test]
fn adc_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 215, 115], OperandSize::Word)
}

#[test]
fn adc_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 23, 77], OperandSize::Word)
}

#[test]
fn adc_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 209, 79], OperandSize::Dword)
}

#[test]
fn adc_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 20, 78, 28], OperandSize::Dword)
}

#[test]
fn adc_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 214, 77], OperandSize::Qword)
}

#[test]
fn adc_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1823366923, Some(OperandSize::Dword), None)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 20, 157, 11, 95, 174, 108, 0], OperandSize::Qword)
}

#[test]
fn adc_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RBP)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 213, 100], OperandSize::Qword)
}

#[test]
fn adc_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RBX, Two, 531993435, Some(OperandSize::Qword), None)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 20, 93, 91, 147, 181, 31, 36], OperandSize::Qword)
}

