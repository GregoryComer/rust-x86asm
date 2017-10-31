use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 218], OperandSize::Word)
}

#[test]
fn adc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 81, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 83, 81], OperandSize::Word)
}

#[test]
fn adc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 209], OperandSize::Dword)
}

#[test]
fn adc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 15], OperandSize::Dword)
}

#[test]
fn adc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 202], OperandSize::Qword)
}

#[test]
fn adc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 19], OperandSize::Qword)
}

#[test]
fn adc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 201], OperandSize::Qword)
}

#[test]
fn adc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1903345272, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 156, 185, 120, 190, 114, 113], OperandSize::Qword)
}

#[test]
fn adc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 231], OperandSize::Word)
}

#[test]
fn adc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 51], OperandSize::Word)
}

#[test]
fn adc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 222], OperandSize::Dword)
}

#[test]
fn adc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 52, 73], OperandSize::Dword)
}

#[test]
fn adc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 237], OperandSize::Qword)
}

#[test]
fn adc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 14], OperandSize::Qword)
}

#[test]
fn adc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 237], OperandSize::Word)
}

#[test]
fn adc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(BX, 32564, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 167, 52, 127], OperandSize::Word)
}

#[test]
fn adc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 247], OperandSize::Dword)
}

#[test]
fn adc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1236075670, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 148, 67, 150, 4, 173, 73], OperandSize::Dword)
}

#[test]
fn adc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 229], OperandSize::Qword)
}

#[test]
fn adc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RCX, 1491679793, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 185, 49, 58, 233, 88], OperandSize::Qword)
}

#[test]
fn adc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 218], OperandSize::Qword)
}

#[test]
fn adc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 30], OperandSize::Qword)
}

#[test]
fn adc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 219], OperandSize::Word)
}

#[test]
fn adc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 28], OperandSize::Word)
}

#[test]
fn adc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 203], OperandSize::Dword)
}

#[test]
fn adc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 28, 65], OperandSize::Dword)
}

#[test]
fn adc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 210], OperandSize::Qword)
}

#[test]
fn adc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 517056898, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 20, 181, 130, 169, 209, 30], OperandSize::Qword)
}

#[test]
fn adc_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 203], OperandSize::Qword)
}

#[test]
fn adc_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 12, 130], OperandSize::Qword)
}

#[test]
fn adc_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 221], OperandSize::Word)
}

#[test]
fn adc_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 3588, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 138, 4, 14], OperandSize::Word)
}

#[test]
fn adc_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 243], OperandSize::Dword)
}

#[test]
fn adc_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 44, 142], OperandSize::Dword)
}

#[test]
fn adc_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 252], OperandSize::Qword)
}

#[test]
fn adc_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DX)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 18], OperandSize::Qword)
}

#[test]
fn adc_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 238], OperandSize::Word)
}

#[test]
fn adc_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 37, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 89, 37], OperandSize::Word)
}

#[test]
fn adc_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 255], OperandSize::Dword)
}

#[test]
fn adc_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 633653590, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 12, 181, 86, 201, 196, 37], OperandSize::Dword)
}

#[test]
fn adc_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 214], OperandSize::Qword)
}

#[test]
fn adc_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 326579296, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 180, 153, 96, 52, 119, 19], OperandSize::Qword)
}

#[test]
fn adc_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RBP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 205], OperandSize::Qword)
}

#[test]
fn adc_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 19, 52, 202], OperandSize::Qword)
}

#[test]
fn adc_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 68], OperandSize::Word)
}

#[test]
fn adc_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 90], OperandSize::Dword)
}

#[test]
fn adc_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 54], OperandSize::Qword)
}

#[test]
fn adc_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(10085)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 101, 39], OperandSize::Word)
}

#[test]
fn adc_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(15440)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 80, 60], OperandSize::Dword)
}

#[test]
fn adc_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(10769)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 17, 42], OperandSize::Qword)
}

#[test]
fn adc_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(2146708812)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 76, 45, 244, 127], OperandSize::Word)
}

#[test]
fn adc_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(806400591)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 79, 178, 16, 48], OperandSize::Dword)
}

#[test]
fn adc_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1517531006)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 126, 175, 115, 90], OperandSize::Qword)
}

#[test]
fn adc_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RAX)), operand2: Some(Literal32(984452203)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 21, 107, 140, 173, 58], OperandSize::Qword)
}

#[test]
fn adc_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 210, 14], OperandSize::Word)
}

#[test]
fn adc_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(SI, 21936, Some(OperandSize::Byte), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 148, 176, 85, 75], OperandSize::Word)
}

#[test]
fn adc_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 211, 47], OperandSize::Dword)
}

#[test]
fn adc_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1927381302, Some(OperandSize::Byte), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 20, 253, 54, 129, 225, 114, 65], OperandSize::Dword)
}

#[test]
fn adc_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 209, 101], OperandSize::Qword)
}

#[test]
fn adc_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 635186042, Some(OperandSize::Byte), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 148, 115, 122, 43, 220, 37, 52], OperandSize::Qword)
}

#[test]
fn adc_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 210, 37], OperandSize::Qword)
}

#[test]
fn adc_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 136773930, Some(OperandSize::Byte), None)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 148, 82, 42, 1, 39, 8, 41], OperandSize::Qword)
}

#[test]
fn adc_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Literal16(25590)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 214, 246, 99], OperandSize::Word)
}

#[test]
fn adc_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Literal16(32149)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 23, 149, 125], OperandSize::Word)
}

#[test]
fn adc_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Literal16(6752)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 213, 96, 26], OperandSize::Dword)
}

#[test]
fn adc_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1887279413, Some(OperandSize::Word), None)), operand2: Some(Literal16(11577)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 148, 203, 53, 153, 125, 112, 57, 45], OperandSize::Dword)
}

#[test]
fn adc_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Literal16(3919)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 213, 79, 15], OperandSize::Qword)
}

#[test]
fn adc_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal16(18993)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 20, 214, 49, 74], OperandSize::Qword)
}

#[test]
fn adc_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Literal32(1138297303)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 215, 215, 9, 217, 67], OperandSize::Word)
}

#[test]
fn adc_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(SI, 23785, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1848445945)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 148, 233, 92, 249, 11, 45, 110], OperandSize::Word)
}

#[test]
fn adc_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1867580573)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 214, 157, 4, 81, 111], OperandSize::Dword)
}

#[test]
fn adc_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 47328502, Some(OperandSize::Dword), None)), operand2: Some(Literal32(637956228)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 148, 113, 246, 44, 210, 2, 132, 112, 6, 38], OperandSize::Dword)
}

#[test]
fn adc_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Literal32(1528037526)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 211, 150, 0, 20, 91], OperandSize::Qword)
}

#[test]
fn adc_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal32(794986778)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 20, 194, 26, 137, 98, 47], OperandSize::Qword)
}

#[test]
fn adc_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RDX)), operand2: Some(Literal32(1219934468)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 210, 4, 185, 182, 72], OperandSize::Qword)
}

#[test]
fn adc_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Literal32(919952686)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 17, 46, 93, 213, 54], OperandSize::Qword)
}

#[test]
fn adc_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DX)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 210, 56], OperandSize::Word)
}

#[test]
fn adc_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 21, 20], OperandSize::Word)
}

#[test]
fn adc_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 215, 66], OperandSize::Dword)
}

#[test]
fn adc_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 20, 67, 48], OperandSize::Dword)
}

#[test]
fn adc_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 213, 122], OperandSize::Qword)
}

#[test]
fn adc_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1273511297, Some(OperandSize::Word), None)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 20, 253, 129, 61, 232, 75, 109], OperandSize::Qword)
}

#[test]
fn adc_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 209, 60], OperandSize::Word)
}

#[test]
fn adc_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 150, Some(OperandSize::Dword), None)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 144, 150, 0, 121], OperandSize::Word)
}

#[test]
fn adc_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 215, 65], OperandSize::Dword)
}

#[test]
fn adc_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 18, 84], OperandSize::Dword)
}

#[test]
fn adc_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 212, 58], OperandSize::Qword)
}

#[test]
fn adc_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RSI, 339722183, Some(OperandSize::Dword), None)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 150, 199, 191, 63, 20, 4], OperandSize::Qword)
}

#[test]
fn adc_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RDI)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 215, 126], OperandSize::Qword)
}

#[test]
fn adc_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RDI, 1355344586, Some(OperandSize::Qword), None)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 151, 202, 234, 200, 80, 0], OperandSize::Qword)
}

